use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m7 15 5 5 5-5" ></ path > < path d = "m7 9 5-5 5 5" ></ path > < / > } } pub const LUCIDE_CHEVRONS_UP_DOWN : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;