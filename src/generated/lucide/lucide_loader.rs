use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "12" y2 = "6" x2 = "12" y1 = "2" ></ line > < line y1 = "18" y2 = "22" x1 = "12" x2 = "12" ></ line > < line x1 = "4.93" y2 = "7.76" x2 = "7.76" y1 = "4.93" ></ line > < line x2 = "19.07" x1 = "16.24" y2 = "19.07" y1 = "16.24" ></ line > < line y2 = "12" x2 = "6" y1 = "12" x1 = "2" ></ line > < line x1 = "18" x2 = "22" y1 = "12" y2 = "12" ></ line > < line y1 = "19.07" x2 = "7.76" x1 = "4.93" y2 = "16.24" ></ line > < line x2 = "19.07" y1 = "7.76" x1 = "16.24" y2 = "4.93" ></ line > < / > } } pub const LucideLoader : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;