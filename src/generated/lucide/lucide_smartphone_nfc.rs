use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" rx = "1" y = "6" height = "12" x = "2" ></ rect > < path d = "M13 8.32a7.43 7.43 0 0 1 0 7.36" ></ path > < path d = "M16.46 6.21a11.76 11.76 0 0 1 0 11.58" ></ path > < path d = "M19.91 4.1a15.91 15.91 0 0 1 .01 15.8" ></ path > < / > } } pub const LucideSmartphoneNfc : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;