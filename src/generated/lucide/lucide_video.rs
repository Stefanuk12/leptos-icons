use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m16 13 5.223 3.482a.5.5 0 0 0 .777-.416V7.87a.5.5 0 0 0-.752-.432L16 10.5" ></ path > < rect y = "6" width = "14" height = "12" rx = "2" x = "2" ></ rect > < / > } } pub const LUCIDE_VIDEO : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;