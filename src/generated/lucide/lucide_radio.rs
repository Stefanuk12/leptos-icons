use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4.9 19.1C1 15.2 1 8.8 4.9 4.9" ></ path > < path d = "M7.8 16.2c-2.3-2.3-2.3-6.1 0-8.5" ></ path > < circle r = "2" cx = "12" cy = "12" ></ circle > < path d = "M16.2 7.8c2.3 2.3 2.3 6.1 0 8.5" ></ path > < path d = "M19.1 4.9C23 8.8 23 15.1 19.1 19" ></ path > < / > } } pub const LUCIDE_RADIO : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;