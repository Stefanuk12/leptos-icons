use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "11" r = "8" cx = "11" ></ circle > < line y2 = "16.65" x2 = "16.65" x1 = "21" y1 = "21" ></ line > < line x1 = "11" y2 = "14" y1 = "8" x2 = "11" ></ line > < line y1 = "11" x2 = "14" y2 = "11" x1 = "8" ></ line > < / > } } pub const LUCIDE_ZOOM_IN : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none")] } ;