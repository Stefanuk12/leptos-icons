use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "10" x1 = "6" y2 = "12" y1 = "12" ></ line > < line y1 = "10" y2 = "14" x2 = "8" x1 = "8" ></ line > < line y2 = "13" x1 = "15" y1 = "13" x2 = "15.01" ></ line > < line x1 = "18" y2 = "11" x2 = "18.01" y1 = "11" ></ line > < rect height = "12" y = "6" width = "20" rx = "2" x = "2" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;