use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" y = "3" x = "3" rx = "2" height = "18" ></ rect > < path d = "M11 9h4a2 2 0 0 0 2-2V3" ></ path > < circle r = "2" cx = "9" cy = "9" ></ circle > < path d = "M7 21v-4a2 2 0 0 1 2-2h4" ></ path > < circle r = "2" cx = "15" cy = "15" ></ circle > < / > } } pub const LUCIDE_CIRCUIT_BOARD : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;