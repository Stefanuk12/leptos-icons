use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "11" cx = "11" r = "8" ></ circle > < line y2 = "16.65" y1 = "21" x1 = "21" x2 = "16.65" ></ line > < line y1 = "8" x1 = "11" x2 = "11" y2 = "14" ></ line > < line x1 = "8" x2 = "14" y2 = "11" y1 = "11" ></ line > < / > } } pub const LUCIDE_ZOOM_IN : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;