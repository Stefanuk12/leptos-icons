use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "15" x = "2" width = "20" rx = "2" y = "4" ></ rect > < rect x = "6" width = "8" y = "8" rx = "1" height = "7" ></ rect > < path d = "M18 8v7" ></ path > < path d = "M6 19v2" ></ path > < path d = "M18 19v2" ></ path > < / > } } pub const LUCIDE_MICROWAVE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;