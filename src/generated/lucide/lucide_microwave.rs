use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "15" x = "2" width = "20" y = "4" rx = "2" ></ rect > < rect y = "8" rx = "1" height = "7" width = "8" x = "6" ></ rect > < path d = "M18 8v7" ></ path > < path d = "M6 19v2" ></ path > < path d = "M18 19v2" ></ path > < / > } } pub const LUCIDE_MICROWAVE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor")] } ;