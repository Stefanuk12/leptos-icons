use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 21h10" ></ path > < rect rx = "2" x = "2" y = "3" height = "14" width = "20" ></ rect > < / > } } pub const LUCIDE_TV_MINIMAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;