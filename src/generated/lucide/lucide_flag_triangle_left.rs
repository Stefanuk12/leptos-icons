use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 22V2L7 7l10 5" ></ path > < / > } } pub const LUCIDE_FLAG_TRIANGLE_LEFT : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24")] } ;