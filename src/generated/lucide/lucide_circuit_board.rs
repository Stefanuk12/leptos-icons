use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" x = "3" height = "18" y = "3" rx = "2" ></ rect > < path d = "M11 9h4a2 2 0 0 0 2-2V3" ></ path > < circle cy = "9" cx = "9" r = "2" ></ circle > < path d = "M7 21v-4a2 2 0 0 1 2-2h4" ></ path > < circle r = "2" cy = "15" cx = "15" ></ circle > < / > } } pub const LUCIDE_CIRCUIT_BOARD : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2")] } ;