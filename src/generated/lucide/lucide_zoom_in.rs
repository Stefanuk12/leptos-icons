use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "8" cy = "11" cx = "11" ></ circle > < line y1 = "21" x2 = "16.65" y2 = "16.65" x1 = "21" ></ line > < line y1 = "8" x1 = "11" y2 = "14" x2 = "11" ></ line > < line x2 = "14" y1 = "11" x1 = "8" y2 = "11" ></ line > < / > } } pub const LUCIDE_ZOOM_IN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;