use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 11v4a2 2 0 0 0 2 2h4" ></ path > < rect rx = "2" x = "13" y = "13" width = "8" height = "8" ></ rect > < / > } } pub const LucideWorkflow : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;