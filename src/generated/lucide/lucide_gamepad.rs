use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "10" x1 = "6" y1 = "12" y2 = "12" ></ line > < line x2 = "8" y2 = "14" x1 = "8" y1 = "10" ></ line > < line x2 = "15.01" x1 = "15" y1 = "13" y2 = "13" ></ line > < line y1 = "11" x1 = "18" x2 = "18.01" y2 = "11" ></ line > < rect rx = "2" height = "12" x = "2" y = "6" width = "20" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;