use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "11" r = "8" cx = "11" ></ circle > < line y1 = "21" x1 = "21" x2 = "16.65" y2 = "16.65" ></ line > < line y2 = "14" x2 = "11" y1 = "8" x1 = "11" ></ line > < line x1 = "8" y2 = "11" x2 = "14" y1 = "11" ></ line > < / > } } pub const LUCIDE_ZOOM_IN : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;