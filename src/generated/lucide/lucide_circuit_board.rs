use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" width = "18" height = "18" rx = "2" y = "3" ></ rect > < path d = "M11 9h4a2 2 0 0 0 2-2V3" ></ path > < circle cx = "9" cy = "9" r = "2" ></ circle > < path d = "M7 21v-4a2 2 0 0 1 2-2h4" ></ path > < circle cx = "15" cy = "15" r = "2" ></ circle > < / > } } pub const LUCIDE_CIRCUIT_BOARD : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;