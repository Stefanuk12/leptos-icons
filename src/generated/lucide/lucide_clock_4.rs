use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < polyline points = "12 6 12 12 16 14" ></ polyline > < / > } } pub const LUCIDE_CLOCK_4 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24")] } ;