use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" rx = "2" y = "3" width = "8" height = "8" ></ rect > < path d = "M7 11v4a2 2 0 0 0 2 2h4" ></ path > < rect x = "13" width = "8" y = "13" height = "8" rx = "2" ></ rect > < / > } } pub const LUCIDE_WORKFLOW : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;