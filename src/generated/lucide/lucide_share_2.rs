use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cx = "18" cy = "5" ></ circle > < circle r = "3" cy = "12" cx = "6" ></ circle > < circle cx = "18" r = "3" cy = "19" ></ circle > < line x2 = "15.42" y1 = "13.51" y2 = "17.49" x1 = "8.59" ></ line > < line x2 = "8.59" y2 = "10.49" x1 = "15.41" y1 = "6.51" ></ line > < / > } } pub const LUCIDE_SHARE_2 : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;