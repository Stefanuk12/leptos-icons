use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "10" ></ circle > < polyline points = "12 6 12 12 14.5 16" ></ polyline > < / > } } pub const LUCIDE_CLOCK_5 : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;