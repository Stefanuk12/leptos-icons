use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" x = "3" width = "18" y = "3" rx = "2" ></ rect > < path d = "M11 9h4a2 2 0 0 0 2-2V3" ></ path > < circle cx = "9" cy = "9" r = "2" ></ circle > < path d = "M7 21v-4a2 2 0 0 1 2-2h4" ></ path > < circle cy = "15" r = "2" cx = "15" ></ circle > < / > } } pub const LucideCircuitBoard : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;