use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "11" cy = "11" r = "8" ></ circle > < line y1 = "21" x2 = "16.65" y2 = "16.65" x1 = "21" ></ line > < line y1 = "8" x2 = "11" x1 = "11" y2 = "14" ></ line > < line y2 = "11" y1 = "11" x1 = "8" x2 = "14" ></ line > < / > } } pub const LUCIDE_ZOOM_IN : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;