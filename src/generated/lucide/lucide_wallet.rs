use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 12V7H5a2 2 0 0 1 0-4h14v4" ></ path > < path d = "M3 5v14a2 2 0 0 0 2 2h16v-5" ></ path > < path d = "M18 12a2 2 0 0 0 0 4h4v-4Z" ></ path > < / > } } pub const LUCIDE_WALLET : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2")] } ;