use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "6" x2 = "12" x1 = "12" y1 = "2" ></ line > < line y1 = "18" x1 = "12" x2 = "12" y2 = "22" ></ line > < line y2 = "7.76" y1 = "4.93" x1 = "4.93" x2 = "7.76" ></ line > < line x2 = "19.07" x1 = "16.24" y2 = "19.07" y1 = "16.24" ></ line > < line x2 = "6" x1 = "2" y1 = "12" y2 = "12" ></ line > < line x2 = "22" y2 = "12" x1 = "18" y1 = "12" ></ line > < line x1 = "4.93" x2 = "7.76" y1 = "19.07" y2 = "16.24" ></ line > < line x1 = "16.24" y1 = "7.76" y2 = "4.93" x2 = "19.07" ></ line > < / > } } pub const LUCIDE_LOADER : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;