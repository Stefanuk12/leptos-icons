use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "11" r = "8" cx = "11" ></ circle > < line y1 = "21" y2 = "16.65" x1 = "21" x2 = "16.65" ></ line > < line x1 = "8" y1 = "11" x2 = "14" y2 = "11" ></ line > < / > } } pub const LUCIDE_ZOOM_OUT : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;