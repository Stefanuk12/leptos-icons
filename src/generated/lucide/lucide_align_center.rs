use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 12H7" ></ path > < path d = "M19 18H5" ></ path > < path d = "M21 6H3" ></ path > < / > } } pub const LUCIDE_ALIGN_CENTER : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;