use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "11" r = "8" cy = "11" ></ circle > < line y2 = "16.65" y1 = "21" x1 = "21" x2 = "16.65" ></ line > < line y2 = "11" x2 = "14" x1 = "8" y1 = "11" ></ line > < / > } } pub const LUCIDE_ZOOM_OUT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24")] } ;