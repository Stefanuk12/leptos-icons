use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "4" cx = "8" cy = "18" ></ circle > < path d = "M12 18V2l7 4" ></ path > < / > } } pub const LUCIDE_MUSIC_2 : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;