use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "18" r = "3" cy = "5" ></ circle > < circle cx = "6" r = "3" cy = "12" ></ circle > < circle cy = "19" r = "3" cx = "18" ></ circle > < line x2 = "15.42" x1 = "8.59" y1 = "13.51" y2 = "17.49" ></ line > < line x2 = "8.59" x1 = "15.41" y1 = "6.51" y2 = "10.49" ></ line > < / > } } pub const LUCIDE_SHARE_2 : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;