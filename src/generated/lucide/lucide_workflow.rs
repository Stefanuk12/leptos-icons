use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "8" y = "3" x = "3" width = "8" ></ rect > < path d = "M7 11v4a2 2 0 0 0 2 2h4" ></ path > < rect y = "13" x = "13" width = "8" height = "8" rx = "2" ></ rect > < / > } } pub const LUCIDE_WORKFLOW : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;