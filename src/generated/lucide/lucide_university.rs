use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "1" cy = "10" ></ circle > < path d = "M22 20V8h-4l-6-4-6 4H2v12a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2" ></ path > < path d = "M6 17v.01" ></ path > < path d = "M6 13v.01" ></ path > < path d = "M18 17v.01" ></ path > < path d = "M18 13v.01" ></ path > < path d = "M14 22v-5a2 2 0 0 0-2-2a2 2 0 0 0-2 2v5" ></ path > < / > } } pub const LUCIDE_UNIVERSITY : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none")] } ;