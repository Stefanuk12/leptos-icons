use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "2" x1 = "12" y2 = "6" x2 = "12" ></ line > < line x1 = "12" y2 = "22" x2 = "12" y1 = "18" ></ line > < line x2 = "7.76" y1 = "4.93" y2 = "7.76" x1 = "4.93" ></ line > < line x1 = "16.24" x2 = "19.07" y1 = "16.24" y2 = "19.07" ></ line > < line x2 = "6" y2 = "12" y1 = "12" x1 = "2" ></ line > < line y1 = "12" x1 = "18" x2 = "22" y2 = "12" ></ line > < line x1 = "4.93" y2 = "16.24" y1 = "19.07" x2 = "7.76" ></ line > < line x2 = "19.07" x1 = "16.24" y2 = "4.93" y1 = "7.76" ></ line > < / > } } pub const LUCIDE_LOADER : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;