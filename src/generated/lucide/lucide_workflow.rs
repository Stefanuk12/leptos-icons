use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "8" y = "3" width = "8" x = "3" ></ rect > < path d = "M7 11v4a2 2 0 0 0 2 2h4" ></ path > < rect y = "13" width = "8" rx = "2" x = "13" height = "8" ></ rect > < / > } } pub const LUCIDE_WORKFLOW : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;