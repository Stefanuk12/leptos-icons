use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M19 15V9" ></ path > < path d = "M15 15h-3v4l-7-7 7-7v4h3v6z" ></ path > < / > } } pub const LUCIDE_ARROW_BIG_LEFT_DASH : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;