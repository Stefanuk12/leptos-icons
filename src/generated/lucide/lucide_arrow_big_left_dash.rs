use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M19 15V9" ></ path > < path d = "M15 15h-3v4l-7-7 7-7v4h3v6z" ></ path > < / > } } pub const LUCIDE_ARROW_BIG_LEFT_DASH : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2")] } ;