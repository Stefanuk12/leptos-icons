use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < polyline points = "12 6 12 12 7.5 12" ></ polyline > < / > } } pub const LUCIDE_CLOCK_9 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;