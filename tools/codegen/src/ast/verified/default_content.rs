use molecule::{create_content, pack_number, write_padding, Number};

pub(crate) trait DefaultContent {
    fn default_content(&self) -> Vec<u8>;
}

impl DefaultContent for super::Option_ {
    fn default_content(&self) -> Vec<u8> {
        Vec::new()
    }
}

impl DefaultContent for super::Union {
    fn default_content(&self) -> Vec<u8> {
        let item_id = 0;
        let inner_content = self.inner[item_id].typ.default_content();
        let total_size = self.header_full_size + inner_content.len();
        let mut content = create_content(total_size);
        content.extend_from_slice(&pack_number(total_size as Number));
        content.extend_from_slice(&pack_number(item_id as Number));
        content.extend_from_slice(&inner_content);
        content
    }
}

impl DefaultContent for super::Array {
    fn default_content(&self) -> Vec<u8> {
        vec![0; self.total_size()]
    }
}

impl DefaultContent for super::Struct {
    fn default_content(&self) -> Vec<u8> {
        vec![0; self.total_size()]
    }
}

impl DefaultContent for super::FixVec {
    fn default_content(&self) -> Vec<u8> {
        let item_count: Number = 0;
        let mut content = create_content(self.header_size);
        content.extend_from_slice(&pack_number(item_count));
        content
    }
}

impl DefaultContent for super::DynVec {
    fn default_content(&self) -> Vec<u8> {
        let item_count: Number = 0;
        let total_size = self.header_base_size as Number;
        let mut content = create_content(self.header_base_size);
        content.extend_from_slice(&pack_number(total_size));
        content.extend_from_slice(&pack_number(item_count));
        content
    }
}

impl DefaultContent for super::Table {
    fn default_content(&self) -> Vec<u8> {
        let field_count = self.inner.len();
        let (total_size, content) = if field_count == 0 {
            let total_size = self.header_size;
            let mut content = create_content(total_size);
            content.extend_from_slice(&pack_number(total_size as Number));
            content.extend_from_slice(&pack_number(field_count as Number));
            (total_size, content)
        } else {
            let (total_size, offsets, field_info) =
                self.field_alignment.iter().zip(self.inner.iter()).fold(
                    (
                        self.header_size,
                        Vec::with_capacity(field_count),
                        Vec::with_capacity(field_count),
                    ),
                    |(mut current_offset, mut offsets, mut field_info), (alignment, field)| {
                        let data = field.typ.default_content();
                        let padding_size = if data.is_empty() {
                            0
                        } else {
                            alignment.calc_padding(current_offset)
                        };
                        current_offset += padding_size;
                        offsets.push(current_offset);
                        current_offset += data.len();
                        field_info.push((padding_size, data));
                        (current_offset, offsets, field_info)
                    },
                );
            let mut content = create_content(total_size);
            content.extend_from_slice(&pack_number(total_size as Number));
            content.extend_from_slice(&pack_number(field_count as Number));
            for offset in offsets.into_iter() {
                content.extend_from_slice(&pack_number(offset as Number));
            }
            for (padding_size, data) in field_info.into_iter() {
                write_padding(&mut content, padding_size).unwrap();
                content.extend_from_slice(&data);
            }
            (total_size, content)
        };
        assert_eq!(content.len(), total_size);
        content
    }
}
