use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" ></ path > < path d = "M12 12v9" ></ path > < path d = "m8 17 4 4 4-4" ></ path > < / > } } pub const LUCIDE_CLOUD_DOWNLOAD : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;