use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < polyline points = "12 6 12 12 9.5 16" ></ polyline > < / > } } pub const LUCIDE_CLOCK_7 : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;