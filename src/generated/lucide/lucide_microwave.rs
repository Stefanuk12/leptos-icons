use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "15" width = "20" y = "4" x = "2" rx = "2" ></ rect > < rect x = "6" rx = "1" width = "8" height = "7" y = "8" ></ rect > < path d = "M18 8v7" ></ path > < path d = "M6 19v2" ></ path > < path d = "M18 19v2" ></ path > < / > } } pub const LUCIDE_MICROWAVE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;