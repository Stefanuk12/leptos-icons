use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "8" y = "3" x = "3" rx = "2" width = "8" ></ rect > < path d = "M7 11v4a2 2 0 0 0 2 2h4" ></ path > < rect height = "8" width = "8" x = "13" y = "13" rx = "2" ></ rect > < / > } } pub const LUCIDE_WORKFLOW : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;