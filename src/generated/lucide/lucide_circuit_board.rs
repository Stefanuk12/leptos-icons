use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 9h4a2 2 0 0 0 2-2V3" ></ path > < circle cx = "9" cy = "9" r = "2" ></ circle > < path d = "M7 21v-4a2 2 0 0 1 2-2h4" ></ path > < circle cx = "15" cy = "15" r = "2" ></ circle > < / > } } pub const LucideCircuitBoard : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;