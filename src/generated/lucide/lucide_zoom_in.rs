use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "11" r = "8" cx = "11" ></ circle > < line x2 = "16.65" x1 = "21" y2 = "16.65" y1 = "21" ></ line > < line y1 = "8" x2 = "11" x1 = "11" y2 = "14" ></ line > < line y2 = "11" x1 = "8" x2 = "14" y1 = "11" ></ line > < / > } } pub const LUCIDE_ZOOM_IN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none")] } ;