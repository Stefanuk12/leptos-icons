use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" ></ path > < circle cx = "9" r = "4" cy = "7" ></ circle > < path d = "M22 21v-2a4 4 0 0 0-3-3.87" ></ path > < path d = "M16 3.13a4 4 0 0 1 0 7.75" ></ path > < / > } } pub const LUCIDE_USERS : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;