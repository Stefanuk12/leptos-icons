use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < polyline points = "12 6 12 12 14.5 8" ></ polyline > < / > } } pub const LUCIDE_CLOCK_1 : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;