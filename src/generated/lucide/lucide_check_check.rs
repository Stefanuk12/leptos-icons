use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 6 7 17l-5-5" ></ path > < path d = "m22 10-7.5 7.5L13 16" ></ path > < / > } } pub const LUCIDE_CHECK_CHECK : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;