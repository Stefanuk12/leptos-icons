use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "12" x1 = "6" y1 = "12" x2 = "10" ></ line > < line y2 = "14" x1 = "8" y1 = "10" x2 = "8" ></ line > < line y1 = "13" x1 = "15" y2 = "13" x2 = "15.01" ></ line > < line y1 = "11" y2 = "11" x2 = "18.01" x1 = "18" ></ line > < rect rx = "2" width = "20" height = "12" y = "6" x = "2" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none")] } ;