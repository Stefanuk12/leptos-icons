use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" x1 = "6" y2 = "12" x2 = "10" ></ line > < line x1 = "8" x2 = "8" y2 = "14" y1 = "10" ></ line > < line y1 = "13" y2 = "13" x2 = "15.01" x1 = "15" ></ line > < line y1 = "11" x1 = "18" x2 = "18.01" y2 = "11" ></ line > < rect rx = "2" height = "12" x = "2" y = "6" width = "20" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;