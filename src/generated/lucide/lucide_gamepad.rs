use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "10" y2 = "12" y1 = "12" x1 = "6" ></ line > < line x2 = "8" y2 = "14" y1 = "10" x1 = "8" ></ line > < line y2 = "13" x2 = "15.01" x1 = "15" y1 = "13" ></ line > < line y2 = "11" x1 = "18" y1 = "11" x2 = "18.01" ></ line > < rect rx = "2" y = "6" width = "20" x = "2" height = "12" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;