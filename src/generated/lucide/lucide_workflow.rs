use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "8" width = "8" y = "3" x = "3" rx = "2" ></ rect > < path d = "M7 11v4a2 2 0 0 0 2 2h4" ></ path > < rect rx = "2" y = "13" height = "8" x = "13" width = "8" ></ rect > < / > } } pub const LUCIDE_WORKFLOW : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;