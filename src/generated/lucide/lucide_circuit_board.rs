use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "3" rx = "2" width = "18" height = "18" ></ rect > < path d = "M11 9h4a2 2 0 0 0 2-2V3" ></ path > < circle cx = "9" cy = "9" r = "2" ></ circle > < path d = "M7 21v-4a2 2 0 0 1 2-2h4" ></ path > < circle r = "2" cx = "15" cy = "15" ></ circle > < / > } } pub const LUCIDE_CIRCUIT_BOARD : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;