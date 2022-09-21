use stylist::Style;

pub fn container() -> Style {
  Style::new(
    r#"
      width: 500px;
      height: 500px;
      display: grid;
      grid-template-columns: repeat(10, 1fr);
      grid-template-rows: repeat(10, 1fr);

      > div {
        border: 1px solid #eee;
        cursor: pointer;
        margin-left: -1px;
        margin-top: -1px;
      }

      > div:hover {
        background: #ddd;
      }
    "#,
  )
  .expect("parse style error")
}
