use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" height = "18" x = "3" width = "18" ></ rect > < path d = "M9 14v1" ></ path > < path d = "M9 19v2" ></ path > < path d = "M9 3v2" ></ path > < path d = "M9 9v1" ></ path > < / > } } pub const LucidePanelLeftDashed : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;